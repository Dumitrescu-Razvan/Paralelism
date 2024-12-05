using System;
using System.Net;
using System.Net.Sockets;
using System.Text;

namespace Lab4.Model;

public class StateObject
{
    public Socket? WorkSocket;
    public const int BufferSize = 1024;
    public byte[] Buffer = new byte[BufferSize];
    public StringBuilder StringBuilder = new StringBuilder();
    public int clientId;
    public string? clientName;
    public string? endpoint;
    public IPEndPoint? ipEndPoint;

    public ManualResetEvent connectDone = new ManualResetEvent(false);
    public ManualResetEvent sendDone = new ManualResetEvent(false);
    public ManualResetEvent receiveDone = new ManualResetEvent(false);

}
