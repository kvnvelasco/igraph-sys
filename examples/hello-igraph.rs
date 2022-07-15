use igraph_sys::*;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_long};
use std::ptr;
use std::ptr::{null, null_mut};

fn main() {
    let mut graph: MaybeUninit<igraph_t> = MaybeUninit::uninit();
    let v: MaybeUninit<igraph_vector_t> = MaybeUninit::uninit();
    let edges: Vec<igraph_real_t> = vec![
        0.0, 1.0, 0.0, 2.0, 0.0, 3.0, 0.0, 4.0, 0.0, 5.0, 0.0, 6.0, 0.0, 7.0, 0.0, 8.0, 0.0, 10.0,
        0.0, 11.0, 0.0, 12.0, 0.0, 13.0, 0.0, 17.0, 0.0, 19.0, 0.0, 21.0, 0.0, 31.0, 1.0, 2.0, 1.0,
        3.0, 1.0, 7.0, 1.0, 13.0, 1.0, 17.0, 1.0, 19.0, 1.0, 21.0, 1.0, 30.0, 2.0, 3.0, 2.0, 7.0,
        2.0, 27.0, 2.0, 28.0, 2.0, 32.0, 2.0, 9.0, 2.0, 8.0, 2.0, 13.0, 3.0, 7.0, 3.0, 12.0, 3.0,
        13.0, 4.0, 6.0, 4.0, 10.0, 5.0, 6.0, 5.0, 10.0, 5.0, 16.0, 6.0, 16.0, 8.0, 30.0, 8.0, 32.0,
        8.0, 33.0, 9.0, 33.0, 13.0, 33.0, 14.0, 32.0, 14.0, 33.0, 15.0, 32.0, 15.0, 33.0, 18.0,
        32.0, 18.0, 33.0, 19.0, 33.0, 20.0, 32.0, 20.0, 33.0, 22.0, 32.0, 22.0, 33.0, 23.0, 25.0,
        23.0, 27.0, 23.0, 32.0, 23.0, 33.0, 23.0, 29.0, 24.0, 25.0, 24.0, 27.0, 24.0, 31.0, 25.0,
        31.0, 26.0, 29.0, 26.0, 33.0, 27.0, 33.0, 28.0, 31.0, 28.0, 33.0, 29.0, 32.0, 29.0, 33.0,
        30.0, 32.0, 30.0, 33.0, 31.0, 32.0, 31.0, 33.0, 32.0, 33.0,
    ];

    let mut result: MaybeUninit<igraph_vector_t> = MaybeUninit::uninit();

    unsafe {
        igraph_vector_view(v.as_ptr(), edges.as_ptr(), edges.len() as c_long);

        igraph_create(
            graph.as_mut_ptr(),
            v.as_ptr(),
            0,
            igraph_i_directed_t::IGRAPH_UNDIRECTED as _,
        );

        igraph_vector_init(result.as_mut_ptr(), 0);
    }

    let mut graph = unsafe { graph.assume_init() };
    let mut result = unsafe { result.assume_init() };

    unsafe {
        igraph_degree(
            &graph,
            &mut result,
            igraph_vss_all(),
            igraph_neimode_t::IGRAPH_ALL,
            1,
        );
    }
    println!(
        "Maximum degree is {}. Vertex {}",
        unsafe { igraph_vector_max(&result) },
        unsafe { igraph_vector_which_max(&result) }
    );

    unsafe {
        igraph_closeness(
            &graph,
            &mut result,
            null_mut(),
            null_mut(),
            igraph_vss_all(),
            igraph_neimode_t::IGRAPH_ALL,
            null(),
            0,
        );
    }
    println!(
        "Maximum closeness is {}, vertex: {}",
        unsafe { igraph_vector_max(&result) },
        unsafe { igraph_vector_which_max(&result) }
    );

    unsafe { igraph_vector_destroy(&mut result) }
    unsafe { igraph_destroy(&mut graph) }
}
