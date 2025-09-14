namespace p5rpc.freecam.BuildScript;

public abstract class Argument
{
    public bool Enabled { get; protected set; } = false;
    public Argument() { }

    public abstract void HandleParams(string[] args);
    public abstract int GetParamCount();
}

public class Debug : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public abstract class ArgumentListBase
{
    public Dictionary<string, Argument> Arguments { get; protected set; }
    protected abstract Dictionary<string, Argument> SetArguments();
    public ArgumentListBase(string[] args)
    {
        Arguments = SetArguments();
        var i = 0;
        while (i < args.Length)
        {
            if (Arguments.ContainsKey(args[i]))
            {
                var arg = Arguments[args[i]];
                arg.HandleParams(args[(i + 1)..(i + 1 + arg.GetParamCount())]);
                i += arg.GetParamCount() + 1;
            }
            else throw new Exception($"Unhandled exception type {args[i]}");
        }
    }

    public Argument this[string k]
    {
        get
        {
            if (Arguments.TryGetValue(k, out var Value))
            {
                return Value;
            }
            else throw new Exception($"Value {k} does not exist in the argument list");
        }
    }
}