using System.Diagnostics;
using System.Reflection;

namespace p5rpc.freecam.BuildScript;

public abstract class ExecutorBase<TArgumentList, TProjectManager>
    where TArgumentList : ArgumentListBase
    where TProjectManager: ProjectManagerBase
{
    public EnvManager EnvManager { get; private set; }
    public ArgumentListBase ArgList { get; private set; }
    public ProjectManagerBase ProjectManager { get; private set; }
    public string RootPath { get; private set; }
    public Stopwatch Watch { get; private set; }
    public abstract string BuildType { get; }
    public ExecutorBase(string[] args)
    {
        RootPath = args[0];
        var pargs = args[1..];
        EnvManager = new(RootPath);
        ConstructorInfo ArgListCtor = typeof(TArgumentList).GetConstructor(
            BindingFlags.Instance | BindingFlags.Public, null, CallingConventions.HasThis, 
            [typeof(string[])], null)!;
        ArgList = (TArgumentList)ArgListCtor.Invoke([pargs]);
        ConstructorInfo ProjectCtor = typeof(TProjectManager).GetConstructor(
            BindingFlags.Instance | BindingFlags.Public, null, CallingConventions.HasThis,
            [typeof(TArgumentList), typeof(string)], null)!;
        ProjectManager = (TProjectManager)ProjectCtor.Invoke([ArgList, RootPath]);
        Watch = new Stopwatch();
    }
    public void PrintInformation()
    {
        Console.WriteLine($"{new ColorRGB(78, 207, 147)}P5R Freecam Build Script{new ClearFormat()}");
        Console.WriteLine($"Build Type: {new BoldFormat()}{BuildType}{new ClearFormat()}");
        using (var gitLog = new Process())
        {
            gitLog.StartInfo.FileName = "git";
            gitLog.StartInfo.Arguments = "log --pretty=format:%H -n 1";
            gitLog.StartInfo.WorkingDirectory = RootPath;
            gitLog.StartInfo.RedirectStandardOutput = true;
            gitLog.Start();

            StreamReader reader = gitLog.StandardOutput;
            Console.WriteLine($"Git Commit: {new BoldFormat()}{reader.ReadToEnd()}{new ClearFormat()}");
            gitLog.WaitForExit();
        }
        Console.WriteLine($"Environment:");
        foreach (var e in EnvManager.EnvValues)
            Console.WriteLine($"  {e.Key}: {new BoldFormat()}{e.Value}{new ClearFormat()}");
        Console.WriteLine($"Arguments:");
        foreach (var k in ArgList.Arguments)
        {
            var color = k.Value.Enabled switch
            {
                true => new ColorRGB(78, 204, 147),
                false => new ColorRGB(237, 66, 155)
            };
            Console.WriteLine($" {k.Key}: {color}{k.Value.Enabled}{new ClearFormat()}");
        }
        Console.WriteLine($"Projects:");
        foreach (var p in ProjectManager.Projects)
            Console.WriteLine($" {new BoldFormat()}{p.Key}{new ClearFormat()} @ {p.Value.RootPath}");
        Console.WriteLine($"{new ColorRGB(78, 204, 147)}MISSION START!{new ClearFormat()}");
        Watch.Start();
    }

    public void PrintCompleted()
    {
        var secs = (double)Watch.ElapsedMilliseconds / 1000;
        var secsFmt = secs switch
        {
            < 60 => $"{secs:0.###} sec",
            >= 60 => $"{(int)(secs / 60)} min, {secs % 60:0.###} sec",
            double.NaN => $"NaN"
        };
        Console.WriteLine($"{new ColorRGB(78, 204, 147)}Success!{new ClearFormat()}");
        Console.WriteLine($"Completed successfuly in {secsFmt}");
    }

    public abstract void Execute();
}