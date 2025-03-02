#include <cstring>
#include <iostream>

namespace sysutil {
template <typename T> T *err_null(T *ptr) {
  if (ptr == nullptr) {
    std::cerr << "Oh dear, something went wrong: " << strerror(errno)
              << std::endl;
    exit(1);
  }
  return ptr;
}

template <typename T> T err_neg(T val) {
  if (val < 0) {
    std::cerr << "Oh dear, something went wrong: " << strerror(errno)
              << std::endl;
    exit(1);
  }
  return val;
}

int get_fd(std::string path);
size_t get_file_len(int fd);

class Mmap {
private:
  std::byte *_ptr;
  size_t _len;

public:
  explicit Mmap(std::string path);
  Mmap(int fd, size_t len);
  explicit Mmap(std::tuple<std::byte *, size_t> memSlice);

  ~Mmap();

  Mmap(const Mmap &) = delete;
  Mmap &operator=(const Mmap &) = delete;
  Mmap(Mmap &&other) { *this = std::move(other); };
  Mmap &operator=(Mmap &&other);

  std::byte *ptr() const { return _ptr; }
  size_t len() const { return _len; }
};

class MemFd {
private:
  int _fd;
  size_t _len;

public:
  int fd() const;
  size_t len() const;

  explicit MemFd(std::tuple<std::byte *, size_t> memSlice);

  ~MemFd();

  MemFd(const MemFd &) = delete;
  MemFd &operator=(const MemFd &) = delete;
  MemFd(MemFd &&other) { *this = std::move(other); };
  MemFd &operator=(MemFd &&other);
};
} // namespace sysutil