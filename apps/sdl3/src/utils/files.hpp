#include <filesystem>
#include <fstream>
#include <vector>

static auto readBinaryFile(const std::filesystem::path& path) -> std::vector<std::uint8_t> {
  std::ifstream stream(path, std::ios::binary);

  std::vector file(std::filesystem::file_size(path), uint8_t{});
  stream.read(reinterpret_cast<char*>(file.data()), file.size());

  return file;
}
