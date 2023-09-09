#include "LHAPDF/LHAPDF.h"
#include "ndinterp_capi.h"
#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#define PDFSET "NNPDF40_nnlo_as_01180"
#define FLAVOR 1
#define VERBOSE true

using namespace std;

vector<double> string_to_vector(const string& stri_raw, const char *sep) {
    // Take the arrays from the LHAPDF file and make them into arrays

    // Remove square brackets
    string stri = stri_raw.substr(1, stri_raw.length() - 2);

    stringstream ss(stri);
    string element;

    vector<double> numbers;
    while (getline(ss, element, *sep)) {
        numbers.push_back(stod(element));
    }

    return numbers;
}


int main() {
    const int n = 5000000;

    cout << "Compare the interpolation results between ndinterp and LHAPDF" << endl;

    LHAPDF::setVerbosity(0);

    LHAPDF::PDF* pdf = LHAPDF::mkPDF(PDFSET, 0);

    cout << "\n > > Comparing alpha_s interpolation" << endl;

    // Read the Q values (can they be obtained in any other way from lhapdf?)
    vector<double> grid_q = string_to_vector(pdf->set().get_entry("AlphaS_Qs"), ",");
    vector<double> als = string_to_vector(pdf->set().get_entry("AlphaS_Vals"), ",");

    for_each(grid_q.begin(), grid_q.end(), [&](double &q) { q = log(q * q); });

    cubic1d* my_grid = create_cubic_interpolator1d(grid_q.data(), als.data(), grid_q.size());

    const double qmin = 3.0;
    const double qmax = 100.0;

    vector<double> q2vals;
    vector<double> ndinterp_results;
    vector<double> lhapdf_results;
    bool ifail = false;

    for (int i = 0; i < n; i++) {
        double rn = (double) rand() / RAND_MAX;
        double q = qmin + rn*(qmax-qmin);
        q2vals.push_back(q*q);
    }

    cout << "Benchmarking the timing!" << endl;
    chrono::steady_clock::time_point st, et;

    st = chrono::steady_clock::now();
    for (double q2: q2vals) {
        double lh_res = pdf->alphasQ2(q2);
        lhapdf_results.push_back(lh_res);
    }
    et = chrono::steady_clock::now();
    chrono::duration<double> lhapdf_time = chrono::duration_cast< chrono::duration<double> >(et - st);
    cout << "while LHAPDF took " << lhapdf_time.count() << " seconds" << endl;

    st = chrono::steady_clock::now();
    for (double q2: q2vals) {
        double my_res = interpolate_cubic_1d(my_grid, log(q2));
        ndinterp_results.push_back(my_res);
    }
    et = chrono::steady_clock::now();
    chrono::duration<double> ndinterp_time = chrono::duration_cast< chrono::duration<double> >(et - st);
    cout << "ndinterp took " << ndinterp_time.count() << " seconds" << endl;

    cout << "Checking whether the results agree" << endl;
    // Now compare the results
    for (int i = 0; i < q2vals.size(); i++) {
        if( abs(ndinterp_results[i]-lhapdf_results[i])/ndinterp_results[i] > 1e-8 ) {
            if (VERBOSE) cout << "Error for q=" << q2vals[i] << endl;
            ifail = true;
        }
    }
    if (!ifail) {
        cout << "All tested points agreed with LHAPDF ✅" << endl;
    }

    delete_cubic_interpolator1d(my_grid);


    cout << "\n > > Comparing PDF interpolation" << endl;

    // Now read the PDF values from one of the `.dat` files
    string lhadata = LHAPDFDATADIR + (string) "/" + PDFSET + (string) "/";
    string pdffile = PDFSET + (string) "_0000.dat";
    string data_file = lhadata + pdffile;
    cout << data_file << endl;

    ifstream datfile(data_file);
    string line;

    // Skip the first 3 lines
    for (int i = 0; i < 3; i++) {
        getline(datfile, line);
    }

    // Read the xgrid
    getline(datfile, line);
    vector<double> grid_x_pdf = string_to_vector(line, " ");
    grid_x_pdf[0] = 1e-9;

    // Read the qgrid
    getline(datfile, line);
    vector<double> grid_q_pdf = string_to_vector(line, " ");

    // Create a grid of PDF values
    vector<double> pdfvals;
    for (double x: grid_x_pdf) {
        for (double q: grid_q_pdf) {
            pdfvals.push_back(pdf->xfxQ2(FLAVOR, x, q*q));
        }
    }

    // Create the array of points to test
    double pdf_qmin = grid_q_pdf[1];
    double pdf_qmax = grid_q_pdf[grid_q_pdf.size() - 2];

    vector<double> q2vals_pdf;
    vector<double> xvals_pdf;

    for (int i = 0; i < n; i++) {
        double rnx = (double) rand() / RAND_MAX;
        double rnq = (double) rand() / RAND_MAX;

        double q = pdf_qmin + rnq*(pdf_qmax-pdf_qmin);
        double x = 1.0e-8 + rnq*0.95;

        q2vals_pdf.push_back(q*q);
        xvals_pdf.push_back(x);
    }

    // Now prepare the 2d grid
    for_each(grid_q_pdf.begin(), grid_q_pdf.end(), [&](double &q) { q = log(q * q); });
    for_each(grid_x_pdf.begin(), grid_x_pdf.end(), [&](double &x) { x = log(x); });
    cubic2d* pdf_grid = create_cubic_interpolator2d(grid_x_pdf.data(), grid_q_pdf.data(), pdfvals.data(), grid_x_pdf.size(), grid_q_pdf.size());



    cout << "Benchmarking the timing!" << endl;

    vector<double> lhapdf_results_pdf;
    vector<double> ndinterp_results_pdf;
    ifail = false;

    st = chrono::steady_clock::now();
    for (int i = 0; i < n; i++) {
        double lh_res = pdf->xfxQ2(FLAVOR, xvals_pdf[i], q2vals_pdf[i]);
        lhapdf_results_pdf.push_back(lh_res);
    }
    et = chrono::steady_clock::now();
    lhapdf_time = chrono::duration_cast< chrono::duration<double> >(et - st);
    cout << "while LHAPDF took " << lhapdf_time.count() << " seconds" << endl;

    st = chrono::steady_clock::now();
    for (int i = 0; i < n; i++) {
        double x = log(xvals_pdf[i]);
        double q = log(q2vals_pdf[i]);
        double my_res = interpolate_cubic_2d(pdf_grid, x, q);
        ndinterp_results_pdf.push_back(my_res);
    }
    et = chrono::steady_clock::now();
    ndinterp_time = chrono::duration_cast< chrono::duration<double> >(et - st);
    cout << "ndinterp took " << ndinterp_time.count() << " seconds" << endl;

    cout << "Checking whether the results agree" << endl;
    // Now compare the results
    for (int i = 0; i < n; i++) {
        if( abs(ndinterp_results_pdf[i]-lhapdf_results_pdf[i])/ndinterp_results_pdf[i] > 4e-3 ) {
            if (VERBOSE) {
                cout << "Error for q=" << q2vals_pdf[i] << " x=" << xvals_pdf[i] << endl;
                cout << "     LHA: " << lhapdf_results_pdf[i] << " ndinterp: " << ndinterp_results_pdf[i] << endl;
            }
            ifail = true;
        }
    }
    if (!ifail) {
        cout << "All tested points agreed with LHAPDF ✅" << endl;
    }

    return 0;
}
