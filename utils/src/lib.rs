//! 一些零散的函数和定义。

#![no_std]
#![deny(warnings, missing_docs)]

/// 打印一些测试信息。
pub fn test_log() {
    use output::*;
    println!(
        r"
  ______        __                _         __
 /_  __/__  __ / /_ ____   _____ (_)____ _ / /
  / /  / / / // __// __ \ / ___// // __ `// /
 / /  / /_/ // /_ / /_/ // /   / // /_/ // /
/_/   \__,_/ \__/ \____//_/   /_/ \__,_//_/
==========================================="
    );
    log::trace!("LOG TEST >> Hello, world!");
    log::debug!("LOG TEST >> Hello, world!");
    log::info!("LOG TEST >> Hello, world!");
    log::warn!("LOG TEST >> Hello, world!");
    log::error!("LOG TEST >> Hello, world!");
    println!();
}

/// 应用程序元数据
#[repr(C)]
pub struct AppMeta {
    base: u64,
    step: u64,
    count: u64,
    first: u64,
}

impl AppMeta {
    /// 以静态链接模式遍历应用程序。
    #[inline]
    pub fn iter_static(&'static self) -> StaticAppIterator {
        StaticAppIterator { meta: self, i: 0 }
    }

    /// 以静态链接模式遍历应用程序。
    #[inline]
    pub fn iter_elf(&'static self) -> ElfIterator {
        ElfIterator { meta: self, i: 0 }
    }
}

/// 静态链接程序迭代器。
pub struct StaticAppIterator {
    meta: &'static AppMeta,
    i: u64,
}

impl Iterator for StaticAppIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.meta.count {
            None
        } else {
            let i = self.i as usize;
            self.i += 1;
            unsafe {
                let slice = core::slice::from_raw_parts(
                    &self.meta.first as *const _ as *const usize,
                    (self.meta.count + 1) as _,
                );
                let pos = slice[i];
                let size = slice[i + 1] - pos;
                let base = self.meta.base as usize + i * self.meta.step as usize;
                core::ptr::copy_nonoverlapping::<u8>(pos as _, base as _, size);
                core::slice::from_raw_parts_mut(base as *mut u8, 0x20_0000)[size..].fill(0);
                Some(base)
            }
        }
    }
}

/// Elf 程序迭代器。
pub struct ElfIterator {
    meta: &'static AppMeta,
    i: u64,
}

impl Iterator for ElfIterator {
    type Item = &'static [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.meta.count {
            None
        } else {
            let i = self.i as usize;
            self.i += 1;
            unsafe {
                let slice = core::slice::from_raw_parts(
                    &self.meta.first as *const _ as *const usize,
                    (self.meta.count + 1) as _,
                );
                let pos = slice[i];
                let size = slice[i + 1] - pos;
                Some(core::slice::from_raw_parts(pos as _, size))
            }
        }
    }
}
