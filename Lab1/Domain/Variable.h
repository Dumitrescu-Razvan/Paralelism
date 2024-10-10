#pragma once
#include <vector>


class Variable
{
private:
    int value;
    std::vector<Variable*> inputs;
    std::vector<Variable*> dependcies;
protected:
public:
    // Constructor and Destructor
    Variable(int value);
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
};
