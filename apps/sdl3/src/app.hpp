#pragma once

#include <span>
#include <string_view>

class App {
public:
  explicit App(std::span<std::string_view> args);
  ~App() = default;

  App(const App&) = delete;
  App(App&&) = delete;
  App& operator=(const App&) = delete;
  App& operator=(App&&) = delete;

  auto run() -> void;

private:
  std::span<std::string_view> args;
};
