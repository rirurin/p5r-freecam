using System.Diagnostics;
using System.IO.Compression;

namespace p5rpc.freecam.BuildScript;

public class PublishState : IDisposable
{
    private string RootPath;
    public string? TempDirectory { get; }

    public string TempDirectoryBuild
    {
        get
        {
            if (TempDirectory != null)
            {
                return Path.Combine(TempDirectory, "Build");
            }
            throw new Exception("TempDirectory must be set before calling TempDirectoryBuild!");
        }
    }

    public string PublishOutputDirectory
    {
        get => Path.Combine(RootPath, "Publish", "ToUpload");
    }
    // User: Publish Output
    public string PublishBuildDirectory // Build directory for current version of the mod.
    {
        get => Path.Combine(RootPath, "Publish", "Builds", "CurrentVersion");
    }

    public string PublishGenericDirectory // Publish files for any target not listed below.
    {
        get => Path.Combine(PublishOutputDirectory, "Generic");
    }
    
    public string PublishNugetDirectory // Publish files for NuGet
    {
        get => Path.Combine(PublishOutputDirectory, "Nuget");
    }

    public string PublishGamebananaDirectory // Publish files for GameBanana
    {
        get => Path.Combine(PublishOutputDirectory, "Gamebanana");
    }
    // Tools
    public string ReloadedToolsPath // Used to check if tools are installed.
    {
        get => Path.Combine(RootPath, "Publish", "Tools", "Reloaded-Tools");
    }

    public string ReloadedPublisherTool
    {
        get => Path.Combine(ReloadedToolsPath, "Reloaded.Publisher.exe");
    }

    public string ChangelogPath
    {
        get => Path.Combine(RootPath, "CHANGELOG.md");
    }

    public PublishState(string _RootPath, bool IsPublishing)
    {
        RootPath = _RootPath;
        if (IsPublishing)
        {
            TempDirectory = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());
            Directory.CreateDirectory(TempDirectory);
            Directory.CreateDirectory(TempDirectoryBuild);
        }
    }

    public void Cleanup()
    {
        if (Directory.Exists(PublishOutputDirectory))
        {
            Directory.Delete(PublishOutputDirectory, true);
        }

        if (Directory.Exists(PublishNugetDirectory))
        {
            Directory.Delete(PublishNugetDirectory, true);
        }

        if (Directory.Exists(PublishGenericDirectory))
        {
            Directory.Delete(PublishGenericDirectory, true);
        }

        if (Directory.Exists(PublishBuildDirectory))
        {
            Directory.Delete(PublishBuildDirectory, true);
        }
    }

    public void GetTools()
    {
        if (TempDirectory == null)
        {
            return;
        }
        Directory.CreateDirectory(ReloadedToolsPath);
        if (Directory.GetFiles(ReloadedToolsPath).Length == 0)
        {
            var toolArchive = Path.Combine(TempDirectory, "Tools.zip");
            Console.WriteLine($"{new BoldFormat()}Downloading Reloaded Tools{new ClearFormat()}");
            File.WriteAllBytes(toolArchive, 
                Utils.DownloadFile("https://github.com/Reloaded-Project/Reloaded-II/releases/latest/download/Tools.zip"));
            ZipFile.ExtractToDirectory(toolArchive, ReloadedToolsPath);
            File.Delete(toolArchive);
        }
    }

    public void CreateArtifacts()
    {
        // Cleanup unnecessary files
        Directory.Delete(TempDirectoryBuild, true);
        foreach (var TargetFile in Directory.GetFiles(PublishBuildDirectory, "*.pdb")
                     .Union(Directory.GetFiles(PublishBuildDirectory, "*.xml")))
        {
            File.Delete(TargetFile);
        }
        // Publish for Generic Target
        Console.WriteLine($"{new ColorRGB(78, 207, 147)}Publish for Generic Target{new ClearFormat()}");
        new PublishGeneric(this).Publish();
        // Publish for Nuget
        Console.WriteLine($"{new ColorRGB(78, 207, 147)}Publish for Nuget{new ClearFormat()}");
        new PublishNuget(this).Publish();
        // Publish for Gamebanana
        Console.WriteLine($"{new ColorRGB(78, 207, 147)}Publish for Gamebanana{new ClearFormat()}");
        new PublishGamebanana(this).Publish();
        // Create published items
        if (TempDirectory != null)
        {
            Directory.Delete(TempDirectory, true);   
        }
    }

    ~PublishState()
    {
        ReleaseUnmanagedResources();
    }

    private void ReleaseUnmanagedResources()
    {
        if (TempDirectory != null)
        {
            Directory.Delete(TempDirectory, true);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }
}

public abstract class PublishCommon
{
    protected PublishState State;
    public PublishCommon(PublishState state)
    {
        State = state;
    }

    public abstract string GetDirectory();

    public abstract string GetPublishTarget();

    public void Publish()
    {
        if (Directory.Exists(GetDirectory()))
        {
            foreach (var InFile in Directory.GetFiles(GetDirectory()))
            {
                File.Delete(InFile);
            }   
        }
        else
        {
            Directory.CreateDirectory(GetDirectory());
        }

        using (var toolPublish = new Process())
        {
            toolPublish.StartInfo.FileName = State.ReloadedPublisherTool;
            toolPublish.StartInfo.Arguments += $"--modfolder \"{State.PublishBuildDirectory}\" ";
            toolPublish.StartInfo.Arguments += "--packagename \"p5rpc.freecam\" ";
            toolPublish.StartInfo.Arguments += $"--changelogpath \"{State.ChangelogPath}\" ";
            toolPublish.StartInfo.Arguments += $"--outputfolder \"{GetDirectory()}\" ";
            toolPublish.StartInfo.Arguments += $"--publishtarget {GetPublishTarget()}";
            toolPublish.Start();
            toolPublish.WaitForExit();
            if (toolPublish.ExitCode != 0)
            {
                Console.WriteLine($"{new ColorRGB(237, 66, 155)}FAILED{new ClearFormat()}");
                throw new Exception($"An error occurred while publishing using target {GetPublishTarget()}, so we can't continue.");
            }
        }
    }
}

public class PublishGeneric : PublishCommon
{
    public PublishGeneric(PublishState state) : base(state) {}

    public override string GetDirectory() => State.PublishGenericDirectory;
    public override string GetPublishTarget() => "Default";
}

public class PublishNuget : PublishCommon
{
    public PublishNuget(PublishState state) : base(state) {}
    
    public override string GetDirectory() => State.PublishNugetDirectory;
    public override string GetPublishTarget() => "NuGet";
}

public class PublishGamebanana : PublishCommon
{
    public PublishGamebanana(PublishState state) : base(state) {}
    
    public override string GetDirectory() => State.PublishGamebananaDirectory;
    public override string GetPublishTarget() => "GameBanana";
}