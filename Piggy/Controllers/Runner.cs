public class Runner : IRunner
{
    private readonly IPrompt prompt;
    private readonly IJobExecutor jobExecutor;

    public Runner(IPrompt prompt, IJobExecutor jobExecutor)
    {
        this.prompt = prompt;
        this.jobExecutor = jobExecutor;
    }

    public void Run(IFile file, string filePath)
    {
        Piggy piggy = file.ReadPiggyConfigurationFile(filePath);

        do
        {
            Job job = prompt.JobSelection(piggy.Jobs);
            if (prompt.ConfirmSelectedJob(job))
            {
                prompt.Print(jobExecutor.Execute(job, piggy.Settings));
            }
        }
        while (prompt.ConfirmContinue());
    }
}

public interface IRunner
{
    void Run(IFile file, string filePath);
}