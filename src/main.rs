use fuser::{Filesystem, MountOption, ReplyAttr, ReplyDirectory, ReplyEmpty, ReplyEntry, ReplyXattr, Request};
use std::env;
use std::ffi::OsStr;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    mountpoint: String,
    #[arg(short, long)]
    debug: bool,
}

struct NullFS;
impl Filesystem for NullFS {
    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.attr(&std::time::Duration::new(0, 0), &fuser::FileAttr {
            ino,
            blksize: 0,
            size: 0,
            blocks: 0,
            atime: std::time::SystemTime::UNIX_EPOCH,
            mtime: std::time::SystemTime::UNIX_EPOCH,
            ctime: std::time::SystemTime::UNIX_EPOCH,
            crtime: std::time::SystemTime::UNIX_EPOCH,
            kind: fuser::FileType::Directory,
            perm: 0o755,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        });
    }
    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        println!("rmdir(parent={}, name={:?})", parent, name);
        reply.ok();
    }

    fn readdir(&mut self, _req: &Request<'_>, ino: u64, fh: u64, offset: i64, reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        reply.ok();
    }

    fn listxattr(&mut self, _req: &Request<'_>, ino: u64, size: u32, reply: ReplyXattr) {
        println!("listxattr(ino={}, size={})", ino, size);
        reply.size(0);
    }

    fn access(&mut self, _req: &Request<'_>, ino: u64, mask: i32, reply: ReplyEmpty) {
        println!("access(ino={}, mask={})", ino, mask);
        reply.ok();
    }

    fn mkdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, umask: u32, reply: ReplyEntry) {
        println!("mkdir(parent={}, name={:?}, mode={}, umask={})", parent, name, mode, umask);
        reply.entry(&std::time::Duration::new(0, 0), &fuser::FileAttr {
            ino: 0,
            blksize: 0,
            size: 0,
            blocks: 0,
            atime: std::time::SystemTime::UNIX_EPOCH,
            mtime: std::time::SystemTime::UNIX_EPOCH,
            ctime: std::time::SystemTime::UNIX_EPOCH,
            crtime: std::time::SystemTime::UNIX_EPOCH,
            kind: fuser::FileType::Directory,
            perm: 0o755,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        }, 0);
    }

    fn mknod(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, umask: u32, rdev: u32, reply: ReplyEntry) {
        println!("mknod(parent={}, name={:?}, mode={}, umask={}, rdev={})", parent, name, mode, umask, rdev);
        reply.entry(&std::time::Duration::new(0, 0), &fuser::FileAttr {
            ino: 0,
            blksize: 0,
            size: 0,
            blocks: 0,
            atime: std::time::SystemTime::UNIX_EPOCH,
            mtime: std::time::SystemTime::UNIX_EPOCH,
            ctime: std::time::SystemTime::UNIX_EPOCH,
            crtime: std::time::SystemTime::UNIX_EPOCH,
            kind: fuser::FileType::Directory,
            perm: 0o755,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        }, 0);
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    println!("mountpoint: {}", args.mountpoint);

    let mountpoint = env::args_os().nth(1).unwrap();
    fuser::mount2(NullFS, mountpoint, &[MountOption::AutoUnmount]).unwrap();
}
