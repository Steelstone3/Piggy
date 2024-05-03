using System.Collections.Generic;
using Xunit;

public class JobFactoryShould
{
    [Fact]
    public void CreateJobs()
    {
        // Given
        List<Job> expectedDefaultJobs = new()
        {
            new Job
            {
                Name = "dotnet-run",
                SubFolder = "",
                Command = "dotnet",
                Arguments = new List<string>() { "run" }
            },
            new Job
            {
                Name = "dotnet-test",
                SubFolder = "ProjectTests",
                Command = "dotnet",
                Arguments = new List<string>() { "test" }
            }
        };
        IJobFactory jobFactory = new JobFactory();

        // When
        List<Job> defaultJobs = jobFactory.Create();

        // Then
        AssertJobEqual(0, expectedDefaultJobs, defaultJobs);
        AssertJobEqual(1, expectedDefaultJobs, defaultJobs);
    }

    private static void AssertJobEqual(int index, List<Job> expectedDefaultJobs, List<Job> defaultJobs)
    {
        Assert.Equal(expectedDefaultJobs[index].Name, defaultJobs[index].Name);
        Assert.Equal(expectedDefaultJobs[index].SubFolder, defaultJobs[index].SubFolder);
        Assert.Equal(expectedDefaultJobs[index].Command, defaultJobs[index].Command);
        Assert.Equal(expectedDefaultJobs[index].Arguments, defaultJobs[index].Arguments);
    }
}