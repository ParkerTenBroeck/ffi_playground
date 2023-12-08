#include <iostream>
#include <string>

#include "extern_api/export.hpp"
#include "extern_api/import.hpp"


int bruh(std::string_view view){
    std::cout << view << "calling from here <-\n";
    return 4;
}

int main(){
    int number = ffi_import::get_integer();
    int number2 = ffi_import::get_integer();
    std::cout << "ptr: " << (void*)bruh << "\n";
    ffi_import::callback_test(bruh);
    
    
    ffi_import::callback_test([number, number2] (std::string_view val) -> int{
        std::cout << number << "\n\n";
        return 44;
    });

    std::string str;
    ffi_import::callback_test([&] (std::string_view val) -> int{
        str += val;
        str += " ";
        number ++;
        number2 ++;
        return 1;
    });
    std::cout << str << "   wowowowowowwowo\n";

    ffi_import::callback_test( [str, number, number2] (std::string_view val) mutable -> int {
        str += val;
        str += "wacked";
        number ++;
        number2 ++;
        return 1;
    });
}
