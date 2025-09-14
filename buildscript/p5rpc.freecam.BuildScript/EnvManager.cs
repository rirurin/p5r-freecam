using YamlDotNet.Core;
using YamlDotNet.Core.Events;

namespace p5rpc.freecam.BuildScript;

public class EnvManager
{
    public static string ENV_LOCAL_PATH = "env.local.yaml";

    public Dictionary<string, string> EnvValues { get; private set; }

    public EnvManager(string root)
    {
        var envPath = Path.Combine(root, ENV_LOCAL_PATH);
        if (!File.Exists(envPath))
            throw new Exception($"File {ENV_LOCAL_PATH} is missing! Please create this, then replace the file paths to point to repositories on your machine!");
        // Load values from ENV local
        using (var envFile = File.OpenText(envPath))
        {
            EnvValues = new();
            var parser = new Parser(envFile);
            parser.Consume<StreamStart>();
            parser.Consume<DocumentStart>();
            parser.Consume<MappingStart>();
            while (parser.Accept<Scalar>(out _))
                EnvValues[parser.Consume<Scalar>().Value] = parser.Consume<Scalar>().Value;
            parser.Consume<MappingEnd>();
            parser.Consume<DocumentEnd>();
            parser.Consume<StreamEnd>();
        }
        // Add to environment variables if they don't exist already
        foreach (var entry in EnvValues)
        {
            var forEnv = entry.Key.Replace("-", "_");
            switch (Environment.GetEnvironmentVariable(forEnv))
            {
                case null:
                    Environment.SetEnvironmentVariable(forEnv, entry.Value);
                    break;
                case var exists:
                    if (exists != forEnv)
                        Environment.SetEnvironmentVariable(forEnv, entry.Value);
                    break;
            }
        }
    }

    public string this[string k]
    {
        get
        {
            if (EnvValues.TryGetValue(k, out var Value))
            {
                return Value;
            }
            else throw new Exception($"Value {k} does not exist in the ENV list");
        }
        set => EnvValues.Add(k, value);
    }
}
