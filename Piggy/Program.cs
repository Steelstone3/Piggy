internal static class Program
{
    internal static void Main()
    {
        string filePath = "piggy.json";
        IPrompt prompt = new Prompt();
        IFile file = new File();
        IFileFinder fileFinder = new FileFinder(prompt);
        
        IJobExecutor jobExecutor = new JobExecutor();
        IRunner runService = new Runner(prompt, jobExecutor);

        filePath = fileFinder.FindFile(file, filePath);

        runService.Run(file, filePath);
    }
}
