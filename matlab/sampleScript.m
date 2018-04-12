close               all;
                    fancyFilteredSamples = zeros(1, length(samples));

for
i                       = 1
:
length(samples)
fancyFilteredSamples(i) = myFancyFilter(b, a, samples(i));
end

%
fancyFilteredSamples                     = filteredSamples(200
:1700);

fMin = min(filteredSamples);
fMax = max(filteredSamples);

%
scaledSamples = 4095 * ((filteredSamples - fMin) / (fMax - fMin));

%
scaledSamples = (scaledSamples / 4095) * 272;

%
plot(filteredSamples);
%
plot(scaledSamples,
'.');