   Compiling fatfs v0.3.6
error[E0463]: can't find crate for `core_io`
  --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/lib.rs:80:1
   |
80 | extern crate core_io;
   | ^^^^^^^^^^^^^^^^^^^^^ can't find crate

error[E0405]: cannot find trait `Read` in this scope
  --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/boot_sector.rs:52:23
   |
52 |     fn deserialize<T: Read>(rdr: &mut T) -> io::Result<BiosParameterBlock> {
   |                       ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/boot_sector.rs:101:21
    |
101 |     fn serialize<T: Write>(&self, mut wrt: T) -> io::Result<()> {
    |                     ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/boot_sector.rs:364:34
    |
364 |     pub(crate) fn deserialize<T: Read>(rdr: &mut T) -> io::Result<BootSector> {
    |                                  ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/boot_sector.rs:379:32
    |
379 |     pub(crate) fn serialize<T: Write>(&self, mut wrt: T) -> io::Result<()> {
    |                                ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Read` in this scope
  --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir.rs:54:28
   |
54 | impl<'a, T: ReadWriteSeek> Read for DirRawStream<'a, T> {
   |                            ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
  --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir.rs:63:28
   |
63 | impl<'a, T: ReadWriteSeek> Write for DirRawStream<'a, T> {
   |                            ^^^^^ not found in this scope
   |
help: consider importing this trait
   |
1  + use alloc::fmt::Write;
   |

error[E0405]: cannot find trait `Seek` in this scope
  --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir.rs:78:28
   |
78 | impl<'a, T: ReadWriteSeek> Seek for DirRawStream<'a, T> {
   |                            ^^^^ not found in this scope

error[E0412]: cannot find type `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir_entry.rs:223:46
    |
223 |     pub(crate) fn serialize(&self, wrt: &mut Write) -> io::Result<()> {
    |                                              ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0412]: cannot find type `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir_entry.rs:287:46
    |
287 |     pub(crate) fn serialize(&self, wrt: &mut Write) -> io::Result<()> {
    |                                              ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0412]: cannot find type `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir_entry.rs:335:46
    |
335 |     pub(crate) fn serialize(&self, wrt: &mut Write) -> io::Result<()> {
    |                                              ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0412]: cannot find type `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/dir_entry.rs:342:41
    |
342 |     pub(crate) fn deserialize(rdr: &mut Read) -> io::Result<Self> {
    |                                         ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/file.rs:186:28
    |
186 | impl<'a, T: ReadWriteSeek> Read for File<'a, T> {
    |                            ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/file.rs:239:28
    |
239 | impl<'a, T: ReadWriteSeek> Write for File<'a, T> {
    |                            ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/file.rs:310:28
    |
310 | impl<'a, T: ReadWriteSeek> Seek for File<'a, T> {
    |                            ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:117:21
    |
117 | pub trait ReadSeek: Read + Seek {}
    |                     ^^^^ not found in this scope

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:117:28
    |
117 | pub trait ReadSeek: Read + Seek {}
    |                            ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:118:9
    |
118 | impl<T: Read + Seek> ReadSeek for T {}
    |         ^^^^ not found in this scope

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:118:16
    |
118 | impl<T: Read + Seek> ReadSeek for T {}
    |                ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:121:26
    |
121 | pub trait ReadWriteSeek: Read + Write + Seek {}
    |                          ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:121:33
    |
121 | pub trait ReadWriteSeek: Read + Write + Seek {}
    |                                 ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:121:41
    |
121 | pub trait ReadWriteSeek: Read + Write + Seek {}
    |                                         ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:122:9
    |
122 | impl<T: Read + Write + Seek> ReadWriteSeek for T {}
    |         ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:122:16
    |
122 | impl<T: Read + Write + Seek> ReadWriteSeek for T {}
    |                ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:122:24
    |
122 | impl<T: Read + Write + Seek> ReadWriteSeek for T {}
    |                        ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:136:23
    |
136 |     fn deserialize<T: Read>(rdr: &mut T) -> io::Result<FsInfoSector> {
    |                       ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:170:21
    |
170 |     fn serialize<T: Write>(&self, wrt: &mut T) -> io::Result<()> {
    |                     ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:601:28
    |
601 | impl<'a, T: ReadWriteSeek> Read for FsIoAdapter<'a, T> {
    |                            ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:607:28
    |
607 | impl<'a, T: ReadWriteSeek> Write for FsIoAdapter<'a, T> {
    |                            ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:621:28
    |
621 | impl<'a, T: ReadWriteSeek> Seek for FsIoAdapter<'a, T> {
    |                            ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:682:26
    |
682 | impl<'a, T: Read + Seek> Read for DiskSlice<T> {
    |                          ^^^^ not found in this scope

error[E0405]: cannot find trait `Read` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:682:13
    |
682 | impl<'a, T: Read + Seek> Read for DiskSlice<T> {
    |             ^^^^ not found in this scope

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:682:20
    |
682 | impl<'a, T: Read + Seek> Read for DiskSlice<T> {
    |                    ^^^^ not found in this scope

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:693:27
    |
693 | impl<'a, T: Write + Seek> Write for DiskSlice<T> {
    |                           ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Write` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:693:13
    |
693 | impl<'a, T: Write + Seek> Write for DiskSlice<T> {
    |             ^^^^^ not found in this scope
    |
help: consider importing this trait
    |
1   + use alloc::fmt::Write;
    |

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:693:21
    |
693 | impl<'a, T: Write + Seek> Write for DiskSlice<T> {
    |                     ^^^^ not found in this scope

error[E0405]: cannot find trait `Seek` in this scope
   --> /root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fatfs-0.3.6/src/fs.rs:714:13
    |
714 | impl<'a, T> Seek for DiskSlice<T> {
    |             ^^^^ not found in this scope

Some errors have detailed explanations: E0405, E0412, E0463.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `fatfs` (lib) due to 37 previous errors
xorriso 1.5.4 : RockRidge filesystem manipulator, libburnia project.

Drive current: -outdev 'stdio:qos-rust.iso'
Media current: stdio file, overwriteable
Media status : is blank
Media summary: 0 sessions, 0 data blocks, 0 data,  698g free
Added to ISO image: directory '/'='/tmp/grub.TOdM1B'
xorriso : UPDATE :     294 files added in 1 seconds
Added to ISO image: directory '/'='/mnt/z/Pulpit/qos-rust/build'
xorriso : UPDATE :     298 files added in 1 seconds
xorriso : NOTE : Copying to System Area: 512 bytes from file '/usr/lib/grub/i386-pc/boot_hybrid.img'
ISO image produced: 2495 sectors
Written to medium : 2495 sectors at LBA 0
Writing to 'stdio:qos-rust.iso' completed successfully.

