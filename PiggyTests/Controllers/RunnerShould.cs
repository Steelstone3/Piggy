using System.Collections.Generic;
using Moq;
using Xunit;

public class RunnerShould
{
    [Fact]
    public void RunOneJobCommand()
    {
        // Given
        string commandOutput = "example";
        string filePath = "piggy.json";
        Job job = new();
        Piggy piggy = new()
        {
            Jobs = new List<Job>() { job },
            Settings = new PiggySettings(),
        };
        Mock<IFile> file = new();
        file.Setup(f => f.ReadPiggyConfigurationFile(filePath)).Returns(piggy);
        Mock<IJobExecutor> jobExecutor = new();
        jobExecutor.Setup(je => je.Execute(job, piggy.Settings)).Returns(commandOutput);
        Mock<IPrompt> prompt = new();
        prompt.Setup(p => p.JobSelection(piggy.Jobs)).Returns(job);
        prompt.Setup(p => p.ConfirmSelectedJob(job)).Returns(true);
        prompt.Setup(p => p.Print(commandOutput));
        prompt.Setup(p => p.ConfirmContinue()).Returns(false);
        IRunner runner = new Runner(prompt.Object, jobExecutor.Object);

        // When
        runner.Run(file.Object, filePath);

        // Then
        file.VerifyAll();
        prompt.VerifyAll();
        jobExecutor.VerifyAll();
    }

    [Fact]
    public void DoNotRunJob()
    {
        // Given
        string filePath = "piggy.json";
        Job job = new();
        Piggy piggy = new();
        Mock<IFile> file = new();
        file.Setup(f => f.ReadPiggyConfigurationFile(filePath)).Returns(piggy);
        Mock<IJobExecutor> jobExecutor = new();
        Mock<IPrompt> prompt = new();
        prompt.Setup(p => p.JobSelection(piggy.Jobs)).Returns(job);
        prompt.Setup(p => p.ConfirmSelectedJob(job)).Returns(false);
        prompt.Setup(p => p.ConfirmContinue()).Returns(false);
        IRunner runner = new Runner(prompt.Object, jobExecutor.Object);

        // When
        runner.Run(file.Object, filePath);

        // Then
        file.VerifyAll();
        prompt.VerifyAll();
    }
}