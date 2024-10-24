#include "Domain/Variable.h"
#include "Checker.h"
#include <iostream>
#include <chrono>
#include <thread>
#include <random>
#include <vector>
#include <unistd.h>
#include <barrier>

#define PRIMARY_VARS 10
#define THREAD_COUNT 100

std::vector<Variable *> GenerateRandomVars(int n)
{

    // 10% of the variables will be primary
    int primary_vars = PRIMARY_VARS;
    std::cout << "Primary vars: " << primary_vars << std::endl;

    std::vector<Variable *> vars;

    // Generate primary variables
    for (int i = 0; i < primary_vars; i++)
    {
        Variable *var = new Variable(std::rand() % 100 + 1);
        vars.push_back(var);
    }

    for (Variable *var : vars)
    {
        std::cout << "Primary Variable: " << var->GetValue() << std::endl;
    }

    /*
        Generate secondary variables
        Each secondary variable will have a random number of inputs
        Each input will be a random primary variable
        Each input will be added to the secondary variable with a 50% chance

    */
    // Generate secondary variables
    for (int i = primary_vars; i < n; i++)
    {
        Variable *var = new Variable(0);

        // Ensure each secondary variable depends on at least one primary variable
        int primary_index = std::rand() % primary_vars;
        // var->AddInput(vars[primary_index]);
        vars[primary_index]->AddDependency(var);

        // Optionally add more inputs from primary or secondary variables
        for (int j = 0; j < primary_vars; j++)
        {
            if (std::rand() % 3 == 0 && primary_index != j)
            {
                // var->AddInput(vars[j]);
                vars[j]->AddDependency(var);
            }
        }
        for (int j = primary_vars; j < i; j++)
        {
            if (std::rand() % 500 == 0 && j != i)
            {
                // var->AddInput(vars[j]);
                vars[j]->AddDependency(var);
            }
        }

        vars.push_back(var);
    }

    for (Variable *var : vars)
    {
        std::cout << "Variable: " << var->GetValue() << " has inputs: ";
        for (Variable *input : var->GetInputs())
        {
            std::cout << input->GetValue() << " ";
        }
        std::cout << std::endl;
    }

    return vars;
}

void randomizeValues(std::vector<Variable*> vars, int i_test) {
    try {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        int randNum = rand() % 100;
        for (int i = 0; i < PRIMARY_VARS; i++) {
            vars[i]->SetValue(std::rand() % 100 + 1);
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
        std::cout << "Thread " << i_test << " finished randomizing values." << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Exception in thread " << i_test << ": " << e.what() << std::endl;
    }
}
void periodCheck(Checker *checker, int period)
{
    while (true)
    {
        int nr = 0;
        if (nr == 10)
        {
            break;
        }
        if (rand() % 4 == 0)
        {
            std::this_thread::sleep_for(std::chrono::seconds(period));
            continue;
        }
        else
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
            nr++;
        }
        checker->lockVariables();
        if (checker->run())
        {
            std::cout << "All variables are consistent" << std::endl;
        }
        else
        {
            std::cout << "Some variables are inconsistent" << std::endl;
        }
        checker->unlockVariables();
    }
}

int main()
{

    // int var_number = std::rand() % 100 + 1;

    // std::vector<Variable*> vars = GenerateRandomVars(var_number);

    // //print vars and their inputs
    // for (Variable* var : vars){
    //     std::cout << "Variable: " << var->GetValue() << " has inputs: " << var->GetInputs().size() << std::endl;
    //     std::cout << "Inputs: ";
    //     for (Variable* input : var->GetInputs()){
    //         std::cout << input->GetValue() << " ";
    //     }
    //     std::cout << std::endl;
    // }

    // Checker* checker = new Checker(vars);

    // if (checker->run()){
    //     std::cout << "All variables are consistent" << std::endl;
    // } else {
    //     std::cout << "Some variables are inconsistent" << std::endl;
    // }

    // return 0;

    int var_number = 100;
    std::vector<Variable *> vars = GenerateRandomVars(var_number);
    Checker *checker = new Checker(vars);

    // for(int i = 0; i < PRIMARY_VARS; i++)
    // {
    //     std::cout << "Variable " << vars[i]->GetValue() << " has dependecies :\n";
    //     for(auto &var: vars[i]->GetDependencies())
    //     {
    //         std::cout << var->GetValue() << " ";
    //     }
    //     std::cout << std::endl;
    // }

    std::thread threads[THREAD_COUNT];

    for (int i = 0; i < THREAD_COUNT; i++)
    {
        threads[i] = std::thread(randomizeValues, vars, i);
    }

    // std::thread checkerThread = std::thread(periodCheck, checker, 100);

    // checkerThread.join();

    if (checker->run())
    {
        std::cout << "All variables are consistent" << std::endl;
    }
    else
    {
        std::cout << "Some variables are inconsistent" << std::endl;
    }

    for (int i = 0; i < THREAD_COUNT; i++)
    {
        threads[i].join();
    }

    return 0;
}