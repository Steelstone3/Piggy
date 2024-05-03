using System.Collections.Generic;
using Xunit;

public class JobShould
{
    [Fact]
    public void Construct()
    {
        // Given
        Job job = new();

        // Then
        Assert.NotNull(job.Name);
        Assert.Empty(job.Name);
        Assert.NotNull(job.SubFolder);
        Assert.Empty(job.SubFolder);
        Assert.NotNull(job.Command);
        Assert.Empty(job.Command);
        Assert.NotNull(job.Arguments);
        Assert.Empty(job.Arguments);
    }

    [Fact]
    public void Display()
    {
        // Given
        string expectedDisplay = "git-add";
        Job job = new Job
        {
            Name = expectedDisplay
        };

        // When
        var display = job.Display();

        // Then
        Assert.Equal(expectedDisplay, display);
    }

    [Fact]
    public void DisplayJobDetail()
    {
        // Given
        string expectedDisplay = "Run: git add --all";
        Job job = new Job
        {
            Command = "git",
            Arguments = new List<string> { "add", "--all" }
        };

        // When
        var display = job.DisplayJobDetail();

        // Then
        Assert.Equal(expectedDisplay, display);
    }
}