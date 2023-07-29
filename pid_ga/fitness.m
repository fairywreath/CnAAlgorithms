function fitnessValue = fitness(parameters)
     % Call the Q2_perfFCN function to get the performance criteria
    [ISE, t_r, t_s, M_p] = Q2_perfFCN(parameters);
    
    % Define the weights for each performance criterion
    weight_ISE = 1.0;             % Weight for ISE
    weight_t_r = 0.0;             % Weight for rise time
    weight_t_s = 5.0;             % Weight for settling time
    weight_M_p = 0.2;             % Weight for maximum overshoot
    
    % Calculate the fitness value as a weighted sum
    fitnessValue = 1 * (weight_ISE * ISE + weight_t_r * t_r + ...
                   weight_t_s * t_s + weight_M_p * M_p);

    %fitnessValue = -1 * (weight_M_p * M_p);
end