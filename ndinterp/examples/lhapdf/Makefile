CXX = c++
CXXFLAGS = -std=c++11
NDFLAGS = -L../../../target/release -lndinterp
LHAFLAGS = $(shell lhapdf-config --libs --cflags)

lharun: lhacheck
	./lhacheck

lhacheck: lhacheck.cpp
	$(CXX) $(CXXFLAGS) -g $< $(NDFLAGS) $(LHAFLAGS) -o $@

clean:
	rm -f lhacheck
