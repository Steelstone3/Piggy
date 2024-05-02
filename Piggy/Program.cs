internal static class Program
{
    internal static void Main()
    {
        IPrompt prompt = new Prompt();
        IJobExecutor jobExecutor = new JobExecutor();
        IRunner runner = new Runner(prompt, jobExecutor);
        IFileFinder fileFinder = new FileFinder(prompt);
        IRunService runService = new RunService(fileFinder, runner);

        string filePath = "piggy.json";
        IFile file = new File();
        runService.Run(file, filePath);
    }
}
