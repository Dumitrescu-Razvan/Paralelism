#pragma once
#include "Domain/Variable.h"
#include <vector>
#include <iostream>

class Checker
{
private:
    std::vector<Variable*> variables;

public:
    Checker(std::vector<Variable*> &vars);
    virtual ~Checker() noexcept = default;
    bool run();
    void lockVariables();
    void unlockVariables();
    bool checkVariables();
};
