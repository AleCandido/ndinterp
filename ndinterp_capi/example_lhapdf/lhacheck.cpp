#include <cstdlib>
#include <iostream>
#include "LHAPDF/LHAPDF.h"
#include <vector>
#include <string>
#include <sstream>

typedef struct cubic1d cubic1d;
extern "C" {
    cubic1d* create_cubic_interpolator1d(const double*, const double*, const size_t);
    double interpolate_cubic_1d(cubic1d*, const double);
}

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

    double qmin = 3.0;
    double qmax = 100.0;

    for (int i = 0; i < 1000; i++) {
        double rn = (double) rand() / RAND_MAX;
        double q = qmin + rn*(qmax-qmin);
        double q2 = q*q;

        double my_res = interpolate_cubic_1d(my_grid, q);
        double lh_res = pdf->alphasQ2(q2);

        if ( (my_res-lh_res)/my_res > 1e-4 ) {
            std::cout << "Error for q=" << q << std::endl;
        }
    }

    return 0;
}
