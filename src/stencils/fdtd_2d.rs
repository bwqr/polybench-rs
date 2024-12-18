use crate::config::stencils::fdtd_2d::DataType;
use crate::ndarray::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array<const NX: usize, const NY: usize, const TMAX: usize>(
    tmax: usize,
    nx: usize,
    ny: usize,
    ex: &mut Array2D<DataType, NX, NY>,
    ey: &mut Array2D<DataType, NX, NY>,
    hz: &mut Array2D<DataType, NX, NY>,
    fict: &mut Array1D<DataType, TMAX>,
) {
    for i in 0..tmax {
        fict[i] = i as DataType;
    }
    for i in 0..nx {
        for j in 0..ny {
            ex[i][j] = (i * (j + 1)) as DataType / nx as DataType;
            ey[i][j] = (i * (j + 2)) as DataType / ny as DataType;
            hz[i][j] = (i * (j + 3)) as DataType / nx as DataType;
        }
    }
}

#[inline(never)]
unsafe fn kernel_fdtd_2d<const NX: usize, const NY: usize, const TMAX: usize>(
    tmax: usize,
    nx: usize,
    ny: usize,
    ex: &mut Array2D<DataType, NX, NY>,
    ey: &mut Array2D<DataType, NX, NY>,
    hz: &mut Array2D<DataType, NX, NY>,
    fict: &Array1D<DataType, TMAX>,
) {
    for t in 0..tmax {
        for j in 0..ny {
            ey[0][j] = fict[t];
        }
        for i in 1..nx {
            for j in 0..ny {
                ey[i][j] = ey[i][j] - 0.5 * (hz[i][j] - hz[i - 1][j]);
            }
        }
        for i in 0..nx {
            for j in 1..ny {
                ex[i][j] = ex[i][j] - 0.5 * (hz[i][j] - hz[i][j - 1]);
            }
        }
        for i in 0..(nx - 1) {
            for j in 0..(ny - 1) {
                hz[i][j] = hz[i][j] - 0.7 * (ex[i][j + 1] - ex[i][j] + ey[i + 1][j] - ey[i][j]);
            }
        }
    }
}

pub fn bench<const NX: usize, const NY: usize, const TMAX: usize>() -> Duration {
    let tmax = TMAX;
    let nx = NX;
    let ny = NY;

    let mut ex = Array2D::<DataType, NX, NY>::uninit();
    let mut ey = Array2D::<DataType, NX, NY>::uninit();
    let mut hz = Array2D::<DataType, NX, NY>::uninit();
    let mut fict = Array1D::<DataType, TMAX>::uninit();

    unsafe {
        init_array(tmax, nx, ny, &mut ex, &mut ey, &mut hz, &mut fict);
        let elapsed =
            util::time_function(|| kernel_fdtd_2d(tmax, nx, ny, &mut ex, &mut ey, &mut hz, &fict));
        util::consume(ex);
        util::consume(ey);
        util::consume(hz);
        elapsed
    }
}

#[test]
fn check() {
    bench::<10, 12, 5>();
}
