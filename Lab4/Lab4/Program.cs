using System;
using System.Net;
using System.Net.Sockets;
using System.Threading;
using Lab4.Implementations;

namespace Lab4;

class Program
{
    static void Main()
    {
        var hosts = new List<string> {
                "www.cs.ubbcluj.ro/~hfpop/teaching/pfl/requirements.html",
                "www.cs.ubbcluj.ro/~forest/newton/index.html",
                "www.cs.ubbcluj.ro/~rlupsa/edu/pdp/index.html"
        };
        Console.WriteLine("DirectCallBack---------------------------------------------");
        DirectCallBack.Run(hosts);

        Console.WriteLine("TaskMechanism----------------------------------------------");
        TaskMechanism.Run(hosts);

        Console.WriteLine("AsyncAwaitTasksMechanism-----------------------------------");
        AsyncAwaitTasksMechanism.Run(hosts);
        Console.ReadLine();

    }
}