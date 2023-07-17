CXX = c++
CXXFLAGS = -std=c++17
# Assuming the lib has been installed with cinstall
NDFLAGS = $(shell pkg-config ndinterp_capi --libs) $(shell pkg-config ndinterp_capi --cflags)
LHAFLAGS = $(shell lhapdf-config --libs --cflags)

lharun: lhacheck
	./lhacheck

lhacheck: lhacheck.cpp
	$(CXX) $(CXXFLAGS) -g $< $(NDFLAGS) $(LHAFLAGS) -o $@ -D LHAPDFDATADIR='"$(shell lhapdf-config --datadir)"'

clean:
	rm -f lhacheck
