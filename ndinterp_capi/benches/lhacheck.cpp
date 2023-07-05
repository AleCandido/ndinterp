#include "LHAPDF/LHAPDF.h"
#include "ndinterp.h"
#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

std::vector<double> string_to_vector(const std::string& stri_raw) {
    // Take the arrays from the LHAPDF file and make them into arrays

    // Remove square brackets
    std::string stri = stri_raw.substr(1, stri_raw.length() - 2);

    std::stringstream ss(stri);
    std::string element;

    std::vector<double> numbers;
    while (std::getline(ss, element, ',')) {
        numbers.push_back(std::stod(element));
    }

    return numbers;
}


int main() {

    std::cout << "Compare the interpolation results between ndinterp and LHAPDF" << std::endl;

    LHAPDF::setVerbosity(0);

    LHAPDF::PDF* pdf = LHAPDF::mkPDF("NNPDF40_nnlo_as_01180", 0);

    // Read the Q values (can they be obtained in any other way from lhapdf?)
    std::vector<double> grid_q = string_to_vector(pdf->set().get_entry("AlphaS_Qs"));
    std::vector<double> als = string_to_vector(pdf->set().get_entry("AlphaS_Vals"));

    std::for_each(grid_q.begin(), grid_q.end(), [&](double &q) { q = std::log(q * q); });

    cubic1d* my_grid = create_cubic_interpolator1d(grid_q.data(), als.data(), grid_q.size());

    const int n = 10000000;
    const double qmin = 3.0;
    const double qmax = 100.0;

    std::vector<double> q2vals;
    std::vector<double> ndinterp_results;
    std::vector<double> lhapdf_results;
    bool ifail = false;

    for (int i = 0; i < n; i++) {
        double rn = (double) rand() / RAND_MAX;
        double q = qmin + rn*(qmax-qmin);
        q2vals.push_back(q*q);
    }

    std::cout << "Benchmarking the timing!" << std::endl;
    std::chrono::steady_clock::time_point st, et;

    st = std::chrono::steady_clock::now();
    for (double q2: q2vals) {
        double lh_res = pdf->alphasQ2(q2);
        lhapdf_results.push_back(lh_res);
    }
    et = std::chrono::steady_clock::now();
    std::chrono::duration<double> lhapdf_time = std::chrono::duration_cast< std::chrono::duration<double> >(et - st);
    std::cout << "while LHAPDF took " << lhapdf_time.count() << " seconds" << std::endl;

    st = std::chrono::steady_clock::now();
    for (double q2: q2vals) {
        double my_res = interpolate_cubic_1d(my_grid, std::log(q2));
        ndinterp_results.push_back(my_res);
    }
    et = std::chrono::steady_clock::now();
    std::chrono::duration<double> ndinterp_time = std::chrono::duration_cast< std::chrono::duration<double> >(et - st);
    std::cout << "ndinterp took " << ndinterp_time.count() << " seconds" << std::endl;

    std::cout << "Checking whether the results agree" << std::endl;
    // Now compare the results
    for (int i = 0; i < q2vals.size(); i++) {
        if( std::abs(ndinterp_results[i]-lhapdf_results[i])/ndinterp_results[i] > 1e-8 ) {
            std::cout << "Error for q=" << q2vals[i] << std::endl;
            ifail = true;
        }
    }
    if (!ifail) {
        std::cout << "All tested points agreed with LHAPDF ✅" << std::endl;
    }

    delete_cubic_interpolator1d(my_grid);

    return 0;
}
