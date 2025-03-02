#include "sysutil.hpp"
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>

namespace sysutil {
int get_fd(std::string path) {
  int fd = err_neg(open(path.c_str(), O_RDONLY));
  return fd;
}
size_t get_file_len(int fd) {
  off_t fsize = err_neg(lseek(fd, 0, SEEK_END));
  return static_cast<size_t>(fsize);
}

Mmap::Mmap(std::string path) {
  int fd = err_neg(open(path.c_str(), O_RDONLY));
  off_t fsize = err_neg(lseek(fd, 0, SEEK_END));
  _len = static_cast<size_t>(fsize);
  _ptr = reinterpret_cast<std::byte *>(
      mmap(nullptr, _len, PROT_READ, MAP_PRIVATE, fd, 0));
  close(fd);
}

Mmap::Mmap(int fd, size_t len) {
  _ptr = reinterpret_cast<std::byte *>(
      mmap(nullptr, _len, PROT_READ, MAP_PRIVATE, fd, 0));
}

Mmap::Mmap(std::tuple<std::byte *, size_t> memSlice) {
  _ptr = std::get<0>(memSlice);
  _len = std::get<1>(memSlice);
}

Mmap &Mmap::operator=(Mmap &&other) {
  this->_ptr = other._ptr;
  this->_len = other._len;
  other._ptr = nullptr;
  other._len = 0;
  return *this;
}

Mmap::~Mmap() {
  if (_ptr != nullptr) {
    err_neg(munmap(_ptr, _len));
    _ptr = nullptr;
    _len = 0;
  }
}

int MemFd::fd() const { return _fd; }

size_t MemFd::len() const { return _len; }

MemFd::MemFd(std::tuple<std::byte *, size_t> memSlice) {
  _fd = err_neg(memfd_create("anyblox::sysutil::MemFd", MFD_ALLOW_SEALING));
  auto [src, len] = memSlice;

  _len = len;
  size_t written = 0;

  while (written < _len) {
    written += err_neg(write(_fd, src + written, len - written));
  }
}

MemFd::~MemFd() {
  if (_fd >= 0) {
    err_neg(close(_fd));
  }
}

MemFd &MemFd::operator=(MemFd &&other) {
  _fd = other._fd;
  _len = other._len;
  other._fd = -1;
  return *this;
}
} // namespace sysutil