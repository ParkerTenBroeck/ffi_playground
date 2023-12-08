#pragma once
#include <string>
#include <functional>

namespace ffi{
    template<typename T>
    struct Array{
        size_t len;
        T* vals;
    };
}


