#include "Variable.h"


Variable::Variable(int value)
{
    this->value = value;
}

Variable::~Variable()
{
}

int Variable::GetValue()
{
    return this->value;
}

std::vector<Variable *> Variable::GetInputs()
{
    return this->inputs;
}

std::vector<Variable *> Variable::GetDependencies()
{
    return this->dependcies;
}

void Variable::AddValue(int value)
{
    this->value += value;
}

void Variable::SubstractValue(int value)
{
    this->value -= value;
}

void Variable::AddInput(Variable *input)
{
    this->inputs.push_back(input);
}

void Variable::AddDependency(Variable *dependency)
{
    this->dependcies.push_back(dependency);
    dependency->AddInput(this);
    dependency->AddValue(this->value);
}

void Variable::SetValue(int value)
{
    int diff = value - this->value;
    this->value = value;
    for (Variable *dependent : this->dependcies)
    {
        dependent->AddValue(diff);  
    }
}