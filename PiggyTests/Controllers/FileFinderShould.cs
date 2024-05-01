using Moq;
using Xunit;

public class FileFinderShould
{
    [Fact]
    public void FindFileUseDefaultFoundFile()
    {
        // Given
        string filePath = "example.json";
        string expectedFilePath = "example.json";
        Mock<IPrompt> prompt = new();
        prompt.Setup(p => p.UseDefault()).Returns(true);
        Mock<IFile> file = new();
        file.Setup(f => f.IsExistingFile(filePath)).Returns(true);
        IFileFinder fileFinder = new FileFinder(prompt.Object);

        // When
        string actualFilePath = fileFinder.FindFile(file.Object, filePath);

        // Then
        prompt.VerifyAll();
        file.VerifyAll();
        Assert.Equal(expectedFilePath, actualFilePath);
    }

    [Fact]
    public void FindFileUseDefaultFileNotFound()
    {
        // Given
        string filePath = "example.json";
        string expectedFilePath = "example.json";
        Mock<IPrompt> prompt = new();
        prompt.Setup(p => p.UseDefault()).Returns(true);
        Mock<IFile> file = new();
        file.Setup(f => f.IsExistingFile(filePath)).Returns(false);
        file.Setup(f => f.CreateDefaultPiggyConfigurationFile());
        IFileFinder fileFinder = new FileFinder(prompt.Object);

        // When
        string actualFilePath = fileFinder.FindFile(file.Object, filePath);

        // Then
        prompt.VerifyAll();
        file.VerifyAll();
        Assert.Equal(expectedFilePath, actualFilePath);
    }
}