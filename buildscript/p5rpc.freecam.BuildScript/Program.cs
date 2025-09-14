namespace p5rpc.freecam.BuildScript;

public class SkipGlobals : Argument
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

public class Timings : Argument
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

public class ArgumentList : ArgumentListBase
{
    public ArgumentList(string[] args) : base(args) { }

    protected override Dictionary<string, Argument> SetArguments()
    {
        return new()
        {
            { "Debug", new Debug() },
            { "SkipGlobals", new SkipGlobals() },
            { "Timings", new Timings() },
        };
    }
}

public class ProjectManager : ProjectManagerBase
{
    public override List<KeyValuePair<string, CodePackage>> GetProjects(ArgumentListBase arg, string RootPath)
    {
        return new List<KeyValuePair<string, CodePackage>>()
        {
            Register(new CSharpProject(arg, Path.Combine(RootPath, "p5rpc.freecam"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "p5r-freecam"))),
        };
    }
    public ProjectManager(ArgumentList arg, string RootPath) : base(arg, RootPath) { }
}

public class Executor : ExecutorBase<ArgumentList, ProjectManager>
{
    public override string BuildType
    {
        get => "CLIENT";
    }

    public Executor(string[] args) : base(args) { }

    public override void Execute()
    {
        PrintInformation();
        // Copy GFD links
        var opengfdBindings = Path.Combine(EnvManager["opengfd-path"], "opengfd-globals/middata/ext_xrd744.rs");
        File.Copy(opengfdBindings, Path.Combine(ProjectManager["p5r-freecam"].RootPath, "src/globals.rs"), true);
        // Create riri_hook folder if it doesn't already exist
        Directory.CreateDirectory(Path.Combine(ProjectManager["p5r-freecam"].RootPath, "riri_hook"));
        // Build P5R Freecam (Rust portion)
        ProjectManager["p5r-freecam"].Build();
        // Build P5R Freecam (C# portion)
        ProjectManager["p5rpc.freecam"].Build();
        // Copy output files from target folder into Reloaded mod
        var reloadedDirectory = Path.Combine(Environment.GetEnvironmentVariable("RELOADEDIIMODS")!, "p5rpc.freecam");
        ((RustCrate)ProjectManager["p5r-freecam"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
        PrintCompleted();
    }
}

internal class Program
{
    static void Main(string[] args)
    {
        if (Environment.GetEnvironmentVariable("RELOADEDIIMODS") == null)
            throw new Exception("The environment variable RELOADEDIIMODS is not defined!");
        var exec = new Executor(args);
        exec.Execute();
    }
}
