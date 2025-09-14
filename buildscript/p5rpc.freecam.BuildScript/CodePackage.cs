using System.Diagnostics;

namespace p5rpc.freecam.BuildScript;

public abstract class CodePackage
{
    public string RootPath { get; private set; }
    public string Name { get; private set; }
    public CodePackage(ArgumentListBase arg, string path)
    {
        RootPath = path;
        Name = Path.GetFileName(RootPath);
    }

    public abstract void Build();
}

public class CSharpProject : CodePackage
{
    public CSharpProject(ArgumentListBase arg, string path) : base(arg, path) { }
    public override void Build()
    {
        using (var crateBuild = new Process())
        {
            crateBuild.StartInfo.FileName = "dotnet";
            crateBuild.StartInfo.Arguments = $"build \"{Name}.csproj\" -v q -c Debug";
            crateBuild.StartInfo.WorkingDirectory = RootPath;
            crateBuild.Start();
            crateBuild.WaitForExit();
            if (crateBuild.ExitCode != 0)
            {
                Console.WriteLine($"{new ColorRGB(237, 66, 155)}FAILED{new ClearFormat()}");
                throw new Exception($"An error occurred while building {Name}, so we can't continue.");
            }
        }
    }
}

public class RustCrate : CodePackage
{
    public string BuildCommand { get; set; }
    public string Profile { get; set; }
    public string BuildStd { get; set; }
    public string CrateType { get; set; }
    public string Target { get; set; }
    public List<string> Features { get; private set; }
    public bool UseDefaultFeatures { get; set; }
    public RustCrate(ArgumentListBase arg, string path) : base(arg, path)
    {
        BuildCommand = "+nightly rustc"; // Build using nightly Rust
        Profile = arg["Debug"].Enabled switch
        {
            true => "slow-debug",
            false => "release"
        };
        BuildStd = "-Z build-std=std,panic_abort";
        CrateType = "cdylib";
        Target = "x86_64-pc-windows-msvc";
        Features = new();
        UseDefaultFeatures = true;
    }

    string GetFeatureList()
    {
        if (Features.Count > 0)
        {
            var FeaturesFmt = $"--features \"";
            foreach (var (Feature, Index) in Features.Select((Feature, index) => (Feature, index)))
            {
                if (Index != 0)
                    FeaturesFmt += ", ";
                FeaturesFmt += Feature;
            }
            FeaturesFmt += "\"";
            return FeaturesFmt;
        }
        else
        {
            return "";
        }
    }
    public override void Build()
    {
        var Cmd = $"{BuildCommand}";
        if (CrateType != "bin")
            Cmd += " --lib";
        Cmd += $" --profile={Profile} {BuildStd}";
        if (CrateType != "bin")
            Cmd += $" --crate-type {CrateType}";
        if (!UseDefaultFeatures)
            Cmd += $" --no-default-features";
        Cmd += $" --target {Target} {GetFeatureList()}";
        Console.WriteLine($"{new BoldFormat()}{Name}{new ClearFormat()}: cargo {Cmd}");
        using (var crateBuild = new Process())
        {
            crateBuild.StartInfo.FileName = "cargo";
            crateBuild.StartInfo.Arguments = Cmd;
            crateBuild.StartInfo.WorkingDirectory = RootPath;
            crateBuild.Start();
            crateBuild.WaitForExit();
            if (crateBuild.ExitCode != 0)
            {
                Console.WriteLine($"{new ColorRGB(237, 66, 155)}FAILED{new ClearFormat()}");
                throw new Exception($"An error occurred while building {Name}, so we can't continue.");
            }
        }
    }

    public void CopyOutputArtifacts(bool IsDebug, string Base, string TargetDirectory)
    {
        var underscoreName = Name.Replace("-", "_");
        var profileFolder = IsDebug switch
        {
            true => "slow-debug",
            false => "release"
        };
        var ArtifactDirectory = Path.Combine(Base, "target", Target, profileFolder);
        var extensions = CrateType switch
        {
            "bin" => new List<string>() { ".exe", ".d" },
            _ => new List<string>() { ".dll", ".dll.lib", ".dll.exp" }
        };
        if (IsDebug) { extensions.Add(".pdb"); }
        foreach (var ext in extensions)
        {
            var targetName = underscoreName;
            if (CrateType == "bin" && ext != ".pdb")
                targetName = Name;
            File.Copy(Path.Combine(ArtifactDirectory, $"{targetName}{ext}"), Path.Combine(TargetDirectory, $"{targetName}{ext}"), true);
        }
    }
}

public abstract class ProjectManagerBase
{
    public Dictionary<string, CodePackage> Projects;

    public KeyValuePair<string, CodePackage> Register<T>(T package) where T : CodePackage
    {
        return new(package.Name, package);
    }
    public abstract List<KeyValuePair<string, CodePackage>> GetProjects(ArgumentListBase arg, string RootPath);
    public ProjectManagerBase(ArgumentListBase arg, string RootPath)
    {
        var ProjectList = GetProjects(arg, RootPath);
        Projects = new(ProjectList);
    }

    public CodePackage this[string k]
    {
        get
        {
            if (Projects.TryGetValue(k, out var Value))
            {
                return Value;
            }
            else throw new Exception($"{k} does not exist in the project list");
        }
    }
}