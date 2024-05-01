internal static class Program
{
    internal static void Main()
    {
        IPrompt prompt = new Prompt();
        IFileFinder fileFinder = new FileFinder(prompt);

        string filePath = "piggy.json";
        IFile file = new File();
        filePath = fileFinder.FindFile(file, filePath);

        IJobExecutor jobExecutor = new JobExecutor();
        IRunner runService = new Runner(prompt, jobExecutor);
        runService.Run(file, filePath);
    }
}
