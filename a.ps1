$AssemblyBuilder = [AppDomain]::CurrentDomain.DefineDynamicAssembly(4, 1);
$ModuleBuilder = $AssemblyBuilder.DefineDynamicModule(2, $False);
$a = $ModuleBuilder.DefineType(0);
[void]$a.DefinePInvokeMethod('GetEditionIdFromName', 'pkeyhelper.dll', 'Public, Static', 1, [int], @([String], [int].MakeByRefType()), 1, 3);
$eid = 0; [void]$a.CreateType()::GetEditionIdFromName((Get-WindowsEdition -Online).Edition, [ref]$eid);
$b = $ModuleBuilder.DefineType(1);
[void]$b.DefinePInvokeMethod('SkuGetProductKeyForEdition', 'pkeyhelper.dll', 'Public, Static', 1, [int], @([int], [String], [String].MakeByRefType(), [String].MakeByRefType()), 1, 3);
$pk = ''; [void]$b.CreateType()::SkuGetProductKeyForEdition($eid, "Volume", [ref]$pk, [ref]$null); $pk