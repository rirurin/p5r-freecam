using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace p5rpc.freecam.BuildScript;

public static class Utils
{
    // Copied from Microsoft documentation :naosmiley:
    // https://learn.microsoft.com/en-us/dotnet/standard/io/how-to-copy-directories
    public static void CopyDirectory(string sourceDir, string destinationDir, bool recursive)
    {
        var dir = new DirectoryInfo(sourceDir); // Get information about the source directory
        if (!dir.Exists) // Check if the source directory exists
            throw new DirectoryNotFoundException($"Source directory not found: {dir.FullName}");
        DirectoryInfo[] dirs = dir.GetDirectories(); // Cache directories before we start copying
        Directory.CreateDirectory(destinationDir); // Create the destination directory
        foreach (FileInfo file in dir.GetFiles())
        { // Get the files in the source directory and copy to the destination directory
            string targetFilePath = Path.Combine(destinationDir, file.Name);
            file.CopyTo(targetFilePath, true);
        }
        if (recursive) // If recursive and copying subdirectories, recursively call this method
        {
            foreach (DirectoryInfo subDir in dirs)
            {
                string newDestinationDir = Path.Combine(destinationDir, subDir.Name);
                CopyDirectory(subDir.FullName, newDestinationDir, true);
            }
        }
    }
}
