#include "Checker.h"

Checker::Checker(std::vector<Variable*> &vars)
{
    variables = vars;
}

bool Checker::run()
{
    lockVariables();
    bool result = checkVariables();
    unlockVariables();
    return result;
}

void Checker::lockVariables()
{
    for (auto &variable : variables)
    {
        variable->mutex.lock();
        variable->LockDependencies();
    }
}

void Checker::unlockVariables()
{
    for (auto &variable : variables)
    {
        variable->mutex.unlock();
        variable->UnlockDependencies();
    }
}

bool Checker::checkVariables()
{
    for(auto &var : variables)
    {
        //std::cout << "Checking variable: " << var->GetValue() << std::endl;
        if (var->GetInputs().size() == 0)
        {
            //std::cout << "Variable: " << var->GetValue() << " is a primary variable" << std::endl;
            continue;
        }
        int sum = 0;
        for (auto &input : var->GetInputs())
        {
            sum += input->GetValue();
        }
        if (sum != var->GetValue())
        {
            // std::cout << "Variable: " << var->GetValue() << " is not consistent with its inputs" << std::endl;
            // std::cout << "It has inputs : " ;
            // for(auto &input : var->GetInputs())
            // {
            //     std::cout << input->GetValue() << " ";
            // }
            // std::cout << std::endl;
            return false;
        }
        // std::cout << "Variable: " << var->GetValue() << " is consistent with its inputs" << std::endl;
    }
    return true;
}
