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
    mutex.lock();
    return this->value;
    mutex.unlock();
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
    this->mutex.lock();
    this->SetValue(this->value + value);
    this->mutex.unlock();
}

void Variable::SubstractValue(int value)
{
    this->value -= value;
}

void Variable::AddInput(Variable *input)
{
    this->inputs.push_back(input);
    this->value += input->GetValue();
}

void Variable::AddDependency(Variable *dependency)
{
    this->dependcies.push_back(dependency);
    dependency->AddInput(this);
    //dependency->AddValue(this->value);
}

void Variable::Lock()
{
    mutex.lock();
    for (Variable *dependent : this->dependcies)
    {
        //dependent->mutex.lock();
        dependent->Lock();
    }
}

void Variable::LockInputs()
{
    for (Variable *input : this->inputs)
    {
        input->mutex.lock();
        input->LockInputs();
    }
}

void Variable::Unlock()
{
    
    mutex.unlock();
    for (Variable *dependent : this->dependcies)
    {
        //dependent->mutex.unlock();
        dependent->Unlock();
    }


}

void Variable::LockDependencies()
{
    for (Variable *dependent : this->dependcies)
    {
        dependent->mutex.lock();
        dependent->LockDependencies();
    }
}

void Variable::UnlockDependencies()
{
    for (Variable *dependent : this->dependcies)
    {
        dependent->mutex.unlock();
        dependent->UnlockDependencies();
    }
}

void Variable::SetValue(int value)
{
    mutex.lock();
    this->LockDependencies();
    int diff = value - this->value;
    // std::cout << "Variable " << GetValue() << " ";
    this->value = value;
    for (Variable *dependent : this->dependcies)
    {
        // std::cout << "Adding value " << diff << " to var " << dependent->GetValue() << " ";
        dependent->AddValue(diff);  
        // std::cout << "New Value : " << dependent->GetValue() << std::endl;
    }
    // std::cout << "Is changin to :" << GetValue() << std::endl;
    mutex.unlock();
    this->UnlockDependencies();

}

