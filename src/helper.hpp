// Helper macroses
#ifndef HELPER_HPP_
#define HELPER_HPP_

#include <fstream>
#include <string_view>
#include <utility>

inline std::pair<std::string, std::string> tokenize(std::string& str, const std::string_view& delim) {
    std::size_t start{};
    std::size_t end = str.find(delim.data());
    std::string key;
    while (end != std::string::npos) {
        key   = str.substr(start, end - start);
        start = end + delim.length();
        end   = str.find(delim.data(), start);
    }
    return {key, str.substr(start, end - start)};
}

inline std::size_t replace_all(std::string& inout, const std::string_view& what, const std::string_view& with) {
    std::size_t count{};
    std::size_t pos{};
    while (std::string::npos != (pos = inout.find(what.data(), pos, what.length()))) {
        inout.replace(pos, what.length(), with.data(), with.length());
        pos += with.length(), ++count;
    }
    return count;
}

inline std::size_t remove_all(std::string& inout, const std::string_view& what) {
    return replace_all(inout, what, "");
}

auto read_whole_file(const std::string_view& path) noexcept -> std::string {
    static constexpr auto read_size = 4096;
    std::ifstream stream{path.data()};
    stream.exceptions(std::ios_base::badbit);

    std::string file{};
    std::string buf(read_size, '\0');
    while (stream.read(&buf[0], read_size)) {
        file.append(buf, 0, stream.gcount());
    }
    file.append(buf, 0, stream.gcount());
    return file;
}

#endif  // HELPER_HPP_
