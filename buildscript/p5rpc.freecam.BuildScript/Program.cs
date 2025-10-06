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

public class Publish : Argument
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
            { "Publish", new Publish() }
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
        if (ArgList["Publish"].Enabled)
        {
            PublishState.Cleanup();
            PublishState.GetTools();
        }
        PrintInformation();
        // Create riri_hook folder if it doesn't already exist
        Directory.CreateDirectory(Path.Combine(ProjectManager["p5r-freecam"].RootPath, "riri_hook"));
        // Copy GFD links
        if (ArgList["Publish"].Enabled)
        {
            // Copy from remote repository - make sure that dependencies have committed up-to-date bindings first!
            var linkFile = Utils.DownloadFile(
                "https://raw.githubusercontent.com/rirurin/opengfd/refs/heads/main/opengfd-globals/middata/ext_xrd744.rs");
            File.WriteAllBytes(Path.Combine(ProjectManager["p5r-freecam"].RootPath, "src/globals.rs"), linkFile);
        }
        else
        {
            // Copy from local environment
            var opengfdBindings = Path.Combine(EnvManager["opengfd-path"], "opengfd-globals/middata/ext_xrd744.rs");
            File.Copy(opengfdBindings, Path.Combine(ProjectManager["p5r-freecam"].RootPath, "src/globals.rs"), true);
        }
        // Build P5R Freecam (Rust portion)
        ProjectManager["p5r-freecam"].Build();
        // Build P5R Freecam (C# portion)
        if (ArgList["Publish"].Enabled)
        {
            ((CSharpProject)ProjectManager["p5rpc.freecam"]).PublishBuildDirectory = PublishState.PublishBuildDirectory;
            ((CSharpProject)ProjectManager["p5rpc.freecam"]).TempDirectory = PublishState.TempDirectoryBuild;
            Directory.CreateDirectory(PublishState.PublishBuildDirectory);
            ((RustCrate)ProjectManager["p5r-freecam"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, 
                RootPath, PublishState.PublishBuildDirectory);
            var modFiles = Path.Combine(ProjectManager["p5r-freecam"].RootPath, "data", "modfiles");
            if (Directory.Exists(modFiles))
            {
                Utils.CopyDirectory(modFiles, PublishState.PublishBuildDirectory, true);
            }
        }
        ProjectManager["p5rpc.freecam"].Build();
        if (ArgList["Publish"].Enabled)
        {
            PublishState.CreateArtifacts();
        }
        else
        {
            // Copy output files from target folder into Reloaded mod
            var reloadedDirectory = Path.Combine(Environment.GetEnvironmentVariable("RELOADEDIIMODS")!, "p5rpc.freecam");
            ((RustCrate)ProjectManager["p5r-freecam"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
            var modFiles = Path.Combine(ProjectManager["p5r-freecam"].RootPath, "data", "modfiles");
            if (Directory.Exists(modFiles))
            {
                Utils.CopyDirectory(modFiles, reloadedDirectory, true);
            }
        }
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
