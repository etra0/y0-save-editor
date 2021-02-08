#ifndef BACKEND_HPP
#define BACKEND_HPP

namespace backend {
extern "C" char *parse_file(const char *name);
extern "C" unsigned int write_savegame(const char *original_file,
                                       const char *modified_json);
extern "C" void free_rust_string(char *parsed);
} // namespace backend

#endif // BACKEND_HPP
