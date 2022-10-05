use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

const PAGE_SIZE: usize = 4096;

pub struct PageId(pub u64);

pub struct DiskManager {
    //ヒープファイルのファイルディスクリプタ
    heap_file: File,
    //採番するページIDを決めるカウンタ
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self { heap_file, next_page_id })
    }

    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file)
    }

    //新しいページIDを採番する
    pub fn allocate_page(&mut self) -> PageId {
        PageId(self.next_page_id += 1)
    }

    fn calc_offset(page_id: PageId) -> u64 {
        PAGE_SIZE * page_id()
    }

    //ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        let offset = PAGE_SIZE * page_id();
        self.heap_file.seek(SeekFrom::Start(calc_offset(page_id())))?;
        self.heap_file.read_exact(data)
    }

    //データをページに書き出す
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        let offset = PAGE_SIZE * page_id();
        self.heap_file.seek(SeekFrom::Start(calc_offset(page_id())))?;
        self.heap_file.write_all()
    }

    pub fn sync(&mut self) -> io::Result<()> {
        self.heap_file.flush()?;
        self.heap_file.sync_all()
    }
}