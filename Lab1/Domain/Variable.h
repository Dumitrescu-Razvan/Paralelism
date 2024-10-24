#pragma once
#include <vector>
#include <mutex>
#include <algorithm>
#include <iostream>

class Variable
{
private:
    int value;
    std::vector<Variable*> inputs;
    std::vector<Variable*> dependcies;

public:
    std::recursive_mutex mutex;
    
    // Constructor and Destructor
    Variable(int value = 0);
    ~Variable();

    // Getters and Setters

    int GetValue();
    std::vector<Variable*> GetInputs();
    std::vector<Variable*> GetDependencies();
    void SetValue(int value);

    // Methods
    void AddValue(int value);
    void SubstractValue(int value);
    void AddInput(Variable* input);
    void AddDependency(Variable* dependency);
    void Lock();
    void LockInputs();
    void Unlock();
    void LockDependencies();
    void UnlockDependencies();

};
