#pragma once

#include <span>
#include <string_view>

class App {
public:
  explicit App(std::span<std::string_view> args);
  ~App() = default;

  App(const App&) = delete;
  App(App&&) = delete;
  auto operator=(const App&) -> App& = delete;
  auto operator=(App&&) -> App& = delete;

  auto run() -> void;

private:
  std::span<std::string_view> args;
};
