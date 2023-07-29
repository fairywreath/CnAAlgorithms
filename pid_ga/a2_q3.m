populationSize = 50;
numGenerations = 150;
crossoverProbability = 0.6;
mutationProbability = 0.25;
KpRange = [2, 18];
TiRange = [1.05, 9.42];
TdRange = [0.26, 2.37];

functionTolerance = 1e-6;

fitnessFunc = @(parameters) fitness(parameters);

options = optimoptions('ga', ...
    'FunctionTolerance', functionTolerance, ...
    'PopulationSize', populationSize, ...
                     'Generations', numGenerations, ...
                     'CrossoverFraction', crossoverProbability, ...
                     'MutationFcn', {@mutationadaptfeasible, mutationProbability}, ...
                    'EliteCount', 2, 'PlotFcns', @gaplotbestf);

%'FunctionTolerance', functionTolerance
[bestIndividual, bestFitness] = ga(fitnessFunc, 3, [], [], [], [], ...
                                  [KpRange(1), TiRange(1), TdRange(1)], ...
                                  [KpRange(2), TiRange(2), TdRange(2)], ...
                                  [], options);

fprintf('Result: Kp=%.2f, Ti=%.2f, Td=%.2f\n', bestIndividual(1), bestIndividual(2), bestIndividual(3));

Kp = bestIndividual(1);
Ti = bestIndividual(2);
Td = bestIndividual(3);

[ISE, t_r, t_s, M_p] = Q2_perfFCN([Kp, Ti, Td])
bestFitness = fitness([Kp, Ti, Td])

G = Kp*tf([Ti*Td, Ti, 1], [Ti, 0]);
F = tf(1, [1, 6, 11, 6, 0]);
sys = feedback(series(G, F), 1);

figure;
step(sys);
grid on;
title('Step Response')
