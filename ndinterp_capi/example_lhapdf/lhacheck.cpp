#include <cstdlib>
#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <chrono>
#include "LHAPDF/LHAPDF.h"
#include "ndinterp.h"


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
    std::vector<double> q2s = string_to_vector(pdf->set().get_entry("AlphaS_Qs"));
    std::vector<double> als = string_to_vector(pdf->set().get_entry("AlphaS_Vals"));

    cubic1d* my_grid = create_cubic_interpolator1d(q2s.data(), als.data(), q2s.size());

    const int n = 10000000;
    const double qmin = 3.0;
    const double qmax = 100.0;

    std::vector<double> qvals;
    std::vector<double> ndinterp_results;
    std::vector<double> lhapdf_results;
    bool ifail = false;

    for (int i = 0; i < n; i++) {
        double rn = (double) rand() / RAND_MAX;
        double q = qmin + rn*(qmax-qmin);
        qvals.push_back(q);
    }

    std::cout << "Benchmarking the timing!" << std::endl;
    std::chrono::steady_clock::time_point st, et;

    st = std::chrono::steady_clock::now();
    for (double q: qvals) {
        double lh_res = pdf->alphasQ2(q*q);
        lhapdf_results.push_back(lh_res);
    }
    et = std::chrono::steady_clock::now();
    std::chrono::duration<double> lhapdf_time = std::chrono::duration_cast< std::chrono::duration<double> >(et - st);
    std::cout << "while LHAPDF took " << lhapdf_time.count() << " seconds" << std::endl;

    st = std::chrono::steady_clock::now();
    for (double q: qvals) {
        double my_res = interpolate_cubic_1d(my_grid, q);
        ndinterp_results.push_back(my_res);
    }
    et = std::chrono::steady_clock::now();
    std::chrono::duration<double> ndinterp_time = std::chrono::duration_cast< std::chrono::duration<double> >(et - st);
    std::cout << "ndinterp took " << ndinterp_time.count() << " seconds" << std::endl;

    std::cout << "Checking whether the results agree" << std::endl;
    // Now compare the results
    for (int i = 0; i < qvals.size(); i++) {
        if( std::abs(ndinterp_results[i]-lhapdf_results[i])/ndinterp_results[i] > 1e-4 ) {
            std::cout << "Error for q=" << qvals[i] << std::endl;
            ifail = true;
        }
    }
    if (!ifail) {
        std::cout << "All tested points agreed with LHAPDF ✅" << std::endl;
    }

    return 0;
}
