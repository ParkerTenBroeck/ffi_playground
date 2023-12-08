#include "common.hpp"

namespace ffi_import{
    extern "C++"{
        void print_integer(int);
        int get_integer();
        ffi::Array<std::string_view> give_rust_slice(std::string_view);
        void callback_test(std::function<int(std::string_view)>);
        // void tokenize
    }
}
