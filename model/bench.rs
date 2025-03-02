use bytemuck::{Pod, Zeroable};
use criterion::{
    black_box, criterion_group, criterion_main, Criterion, Throughput,
};
use que::{
    headless_spmc::{consumer::Consumer, producer::Producer},
    shmem::cleanup_shmem,
    shem::Channel,
    Channel,
};

use nexon::page_size::PageSize;

#[derive(Copy, Clone, Zeroable, PartialEq, Debug)]
pub struct Transaction<const N: usize> {
    pub bytes: [u8; N],
}

unsafe impl Pod for Transaction<1232> {}
