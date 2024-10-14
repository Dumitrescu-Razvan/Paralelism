#include "Domain/Variable.h"
#include <iostream>
int main(){
    Variable v1(10);
    Variable v2(20);
    v1.AddDependency(&v2);
    std::cout << v1.GetValue() << std::endl;
    std::cout << v2.GetValue() << std::endl;
    v1.SetValue(30);
    std::cout << v1.GetValue() << std::endl;
    std::cout << v2.GetValue() << std::endl;
    std::vector<Variable*> inputs = v2.GetInputs();
    for (Variable* input : inputs)
    {
        std::cout << input->GetValue() << std::endl;
    }
    Variable v3(40);
    v3.AddDependency(&v2);
    std::cout << v1.GetValue() << std::endl;
    std::cout << v2.GetValue() << std::endl;
    std::cout << v3.GetValue() << std::endl;
    inputs = v2.GetInputs();
    for (Variable* input : inputs)
    {
        std::cout << input->GetValue() << ',';
    }
}