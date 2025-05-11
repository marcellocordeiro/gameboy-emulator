#pragma once

#include <filesystem>
#include <fstream>
#include <vector>

#include "common.hpp"

static auto read_binary_file(const std::filesystem::path& path) -> std::vector<u8> {
  std::ifstream stream(path, std::ios::binary);

  stream.seekg(0, std::ios_base::end);
  const auto length = stream.tellg();
  stream.seekg(0, std::ios_base::beg);

  if (length < 0) {
    throw std::runtime_error("File length is negative.");
  }

  const auto vec_length = static_cast<usize>(length);

  std::vector file(vec_length, u8{});
  stream.read(reinterpret_cast<char*>(file.data()), length);

  return file;
}
