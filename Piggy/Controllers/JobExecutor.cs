using System;
using System.ComponentModel;
using System.Diagnostics;

public class JobExecutor : IJobExecutor
{
    public string Execute(Job job, PiggySettings piggySettings)
    {
        // Create a new ProcessStartInfo.
        ProcessStartInfo processStartInformation = new ProcessStartInfo
        {
            FileName = job.Command, // Specify the command you want to run.
            Arguments = string.Join(' ', job.Arguments), // Specify any arguments here.
            WorkingDirectory = piggySettings.ProjectFolder,
            RedirectStandardOutput = true,
            UseShellExecute = false
        };

        // Start the process.
        Process process = new Process
        {
            StartInfo = processStartInformation
        };
        process.Start();

        process.WaitForExit();
        
        return process.StandardOutput.ReadToEnd();
    }
}

public interface IJobExecutor {
    string Execute(Job job, PiggySettings piggySettings);
}