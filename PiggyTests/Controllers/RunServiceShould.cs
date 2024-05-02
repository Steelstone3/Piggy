using Moq;
using Xunit;

public class RunServiceShould
{
    [Fact]
    public void Run()
    {
        // Given
        string filePath = "piggy.json";
        Mock<IFile> file = new();
        Mock<IFileFinder> fileFinder = new();
        fileFinder.Setup(ff => ff.FindFile(file.Object, filePath)).Returns(filePath);
        Mock<IRunner> runner = new();
        runner.Setup(r => r.Run(file.Object, filePath));
        IRunService runService = new RunService(fileFinder.Object, runner.Object);

        // When
        runService.Run(file.Object, filePath);

        // Then
        fileFinder.VerifyAll();
        runner.VerifyAll();
    }
}