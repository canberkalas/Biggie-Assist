# PowerShell script to record audio using Windows native capabilities
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
using System.Threading;

public class AudioRecorder
{
    [DllImport("winmm.dll", EntryPoint = "mciSendString")]
    public static extern int MciSendString(string command, string buffer, int bufferSize, IntPtr hwndCallback);

    public void Record()
    {
        MciSendString("open new Type waveaudio Alias recsound", null, 0, IntPtr.Zero);
        MciSendString("record recsound", null, 0, IntPtr.Zero);
    }

    public void Stop(string filePath)
    {
        MciSendString("save recsound " + filePath, null, 0, IntPtr.Zero);
        MciSendString("close recsound", null, 0, IntPtr.Zero);
    }
}
"@

$recorder = [AudioRecorder]::new()
$recorder.Record()
Start-Sleep -Seconds 10
$recorder.Stop("audio_samples/test.wav")
