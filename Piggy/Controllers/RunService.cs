public class RunService : IRunService
{
    private readonly IFileFinder fileFinder;
    private readonly IRunner runner;

    public RunService(IFileFinder fileFinder, IRunner runner)
    {
        this.fileFinder = fileFinder;
        this.runner = runner;
    }

    public void Run(IFile file, string filePath)
    {
        filePath = fileFinder.FindFile(file, filePath);
        runner.Run(file, filePath);
    }
}

public interface IRunService
{
    void Run(IFile file, string filePath);
}